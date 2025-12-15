import { useEffect, useMemo, useState } from 'react'
import './App.css'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'
import { listen } from '@tauri-apps/api/event'

type ExtractItem = { path: string; hash: string; mode: number }

function App() {
  const [files, setFiles] = useState<string[]>([])
  const [extracted, setExtracted] = useState<ExtractItem[]>([])
  const [attack, setAttack] = useState<'dict' | 'brute' | 'mask'>('dict')
  const [dict, setDict] = useState('')
  const [mask, setMask] = useState('')
  const [minLen, setMinLen] = useState(1)
  const [maxLen, setMaxLen] = useState(8)
  const [setD, setSetD] = useState(true)
  const [setL, setSetL] = useState(true)
  const [setU, setSetU] = useState(true)
  const [setS, setSetS] = useState(false)
  const [session, setSession] = useState('epcc_session')
  const [deviceType, setDeviceType] = useState<'cpu'|'gpu'|'auto'>('auto')
  const [status, setStatus] = useState<string[]>([])
  const [err, setErr] = useState('')
  const [hashcatPath, setHashcatPath] = useState('')
  const [office2Path, setOffice2Path] = useState('')
  const [devicesText, setDevicesText] = useState('')
  const [selectedDevices, setSelectedDevices] = useState<number[]>([])

  useEffect(() => {
    const un = listen<string>('hashcat-status', (e) => {
      setStatus((s) => [...s.slice(-500), e.payload])
    })
    return () => { un.then((f) => f()) }
  }, [])

  const pickFiles = async () => {
    try {
      const sel = await open({ multiple: true, filters: [{ name: 'Excel', extensions: ['xlsx', 'xls'] }] })
      const arr = Array.isArray(sel) ? sel : sel ? [sel] : []
      setFiles(arr as string[])
      setErr('')
    } catch (e: any) {
      setErr(String(e?.message || e))
    }
  }

  const preview = useMemo(() => {
    const it = extracted[0]
    if (!it) return ''
    const base = [`-m ${it.mode}`, `--status --status-json --status-timer=1`, `--session ${session}`]
    if (attack === 'dict') {
      if (!dict) return ''
      return `hashcat ${base.join(' ')} -a 0 <hash.txt> ${dict}`
    }
    if (attack === 'mask') {
      if (!mask) return ''
      return `hashcat ${base.join(' ')} -a 3 <hash.txt> ${mask}`
    }
    const sets: string[] = []
    const charset = [setD ? '?d' : '', setL ? '?l' : '', setU ? '?u' : '', setS ? '?s' : ''].filter(Boolean).join('')
    if (charset) sets.push(`-1 ${charset}`)
    const m = Array(Math.max(minLen, 1)).fill('?1').join('')
    return `hashcat ${base.join(' ')} ${sets.join(' ')} -a 3 <hash.txt> ${m} --increment --increment-min ${minLen} --increment-max ${maxLen}`
  }, [extracted, attack, dict, mask, minLen, maxLen, setD, setL, setU, setS, session])

  const doExtract = async () => {
    try {
      if (files.length === 0) { setErr('请先选择 Excel 文件'); return }
      const res = await invoke<[string, string, number][]>('extract_hash', { paths: files, customOffice2: office2Path || null })
      setExtracted(res.map((r) => ({ path: r[0], hash: r[1], mode: r[2] })))
      setErr('')
    } catch (e: any) {
      setErr(String(e?.message || e))
    }
  }

  const start = async () => {
    try {
      const it = extracted[0]
      if (!it) { setErr('请先提取哈希'); return }
      let req: any = { hash: it.hash, mode: it.mode, attack: 0, dict: null, mask: null, incrementMin: null, incrementMax: null, customSets: null, devices: null, deviceType: null, session }
      if (attack === 'dict') {
        req.attack = 0
        req.dict = dict
      } else if (attack === 'mask') {
        req.attack = 3
        req.mask = mask
      } else {
        req.attack = 3
        const cs = [setD ? '?d' : '', setL ? '?l' : '', setU ? '?u' : '', setS ? '?s' : ''].filter(Boolean).join('')
        req.customSets = cs ? [['1', cs]] : null
        req.mask = Array(Math.max(minLen, 1)).fill('?1').join('')
        req.incrementMin = minLen
        req.incrementMax = maxLen
      }
      if (deviceType==='cpu') req.deviceType = 1
      if (deviceType==='gpu') req.deviceType = 2
      if (hashcatPath) req.customHashcat = hashcatPath
      if (selectedDevices.length) req.devices = selectedDevices
      await invoke('start_attack', { req })
      setErr('')
    } catch (e: any) {
      setErr(String(e?.message || e))
    }
  }

  const ctrl = async (a: 'pause' | 'resume' | 'stop') => {
    await invoke('control_attack', { action: a })
  }

  return (
    <div style={{ padding: 16 }}>
      <h2>Excel Password Cracker Client (EPCC)</h2>
      <div>
        <button onClick={pickFiles}>选择文件</button>
        <span style={{ marginLeft: 8 }}>{files.length ? files.join(', ') : ''}</span>
        <button onClick={doExtract} style={{ marginLeft: 12 }} disabled={files.length === 0}>提取哈希</button>
      </div>
      {!!err && (<div style={{ color: '#ff6b6b', marginTop: 8 }}>{err}</div>)}
      {extracted[0] && (
        <div style={{ marginTop: 12 }}>
          <div>哈希模式：{extracted[0].mode}</div>
          <div style={{ marginTop:8 }}>
            Hashcat 路径：<input value={hashcatPath} onChange={(e)=>setHashcatPath(e.target.value)} style={{ width: 500 }} />
          </div>
          <div style={{ marginTop:8 }}>
            office2hashcat 路径：<input value={office2Path} onChange={(e)=>setOffice2Path(e.target.value)} style={{ width: 500 }} />
          </div>
          <div style={{ display: 'flex', gap: 12, marginTop: 8 }}>
            <label><input type="radio" checked={attack==='dict'} onChange={() => setAttack('dict')} /> 字典攻击</label>
            <label><input type="radio" checked={attack==='brute'} onChange={() => setAttack('brute')} /> 纯暴力</label>
            <label><input type="radio" checked={attack==='mask'} onChange={() => setAttack('mask')} /> 掩码攻击</label>
          </div>
          {attack==='dict' && (
            <div style={{ marginTop:8 }}>
              <input placeholder="字典文件路径" value={dict} onChange={(e)=>setDict(e.target.value)} style={{ width: 400 }} />
            </div>
          )}
          {attack==='mask' && (
            <div style={{ marginTop:8 }}>
              <input placeholder="掩码，例如 ?d?d?d?d" value={mask} onChange={(e)=>setMask(e.target.value)} style={{ width: 400 }} />
            </div>
          )}
          {attack==='brute' && (
            <div style={{ marginTop:8 }}>
              <div>
                长度：<input type="number" value={minLen} onChange={(e)=>setMinLen(parseInt(e.target.value)||1)} style={{ width: 60 }} /> - <input type="number" value={maxLen} onChange={(e)=>setMaxLen(parseInt(e.target.value)||8)} style={{ width: 60 }} />
              </div>
              <div style={{ display:'flex', gap:12, marginTop:8 }}>
                <label><input type="checkbox" checked={setD} onChange={()=>setSetD(!setD)} /> 数字 ?d</label>
                <label><input type="checkbox" checked={setL} onChange={()=>setSetL(!setL)} /> 小写 ?l</label>
                <label><input type="checkbox" checked={setU} onChange={()=>setSetU(!setU)} /> 大写 ?u</label>
                <label><input type="checkbox" checked={setS} onChange={()=>setSetS(!setS)} /> 特殊 ?s</label>
              </div>
            </div>
          )}
          <div style={{ marginTop:8 }}>
            设备类型：
            <label><input type="radio" checked={deviceType==='auto'} onChange={()=>setDeviceType('auto')} /> 自动</label>
            <label><input type="radio" checked={deviceType==='cpu'} onChange={()=>setDeviceType('cpu')} /> 仅 CPU</label>
            <label><input type="radio" checked={deviceType==='gpu'} onChange={()=>setDeviceType('gpu')} /> 仅 GPU</label>
          </div>
          <div style={{ marginTop: 12 }}>
            <div>会话名：<input value={session} onChange={(e)=>setSession(e.target.value)} /></div>
          </div>
          <div style={{ marginTop: 12 }}>
            <div>Hashcat 命令预览：</div>
            <textarea value={preview} readOnly style={{ width: 700, height: 80 }} />
          </div>
          <div style={{ marginTop: 12 }}>
            <button onClick={async ()=>{ try { const out = await invoke<string>('list_devices', { customHashcat: hashcatPath || null }); setDevicesText(out); setErr('') } catch (e: any) { setErr(String(e?.message || e)) } }}>检测设备</button>
            {!!devicesText && (
              <div style={{ display:'flex', gap:12, marginTop:8 }}>
                {Array.from(devicesText.matchAll(/Device\s+#(\d+)/g)).map((m)=>{
                  const id = parseInt(m[1]);
                  return (
                    <label key={id}><input type="checkbox" checked={selectedDevices.includes(id)} onChange={()=>setSelectedDevices((prev)=> prev.includes(id)? prev.filter((x)=>x!==id): [...prev, id])} /> 设备 #{id}</label>
                  )
                })}
              </div>
            )}
            <textarea value={devicesText} readOnly style={{ width: 700, height: 160, marginTop:8 }} />
          </div>
          <div style={{ marginTop: 12, display:'flex', gap:8 }}>
            <button onClick={start}>开始</button>
            <button onClick={()=>ctrl('pause')}>暂停</button>
            <button onClick={()=>ctrl('resume')}>恢复</button>
            <button onClick={()=>ctrl('stop')}>停止</button>
            <button onClick={()=>invoke('restore_session')}>恢复会话</button>
          </div>
          <div style={{ marginTop: 12 }}>
            <div>状态：</div>
            <textarea value={status.join('\n')} readOnly style={{ width: 700, height: 180 }} />
          </div>
        </div>
      )}
    </div>
  )
}

export default App
