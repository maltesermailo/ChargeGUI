import { BrowserRouter, Routes, Route, useNavigate } from 'react-router-dom';
import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from 'react';

interface SyscallDefinition {
  id: number;
  name: string;
  description: string;
}

interface Syscall {
  syscall: SyscallDefinition;
  enabled: boolean;
}

interface SyscallsList {
  syscalls: Syscall[];
}


function SyscallPage() {
  const [syscalls, setSyscalls] = useState<SyscallsList>({syscalls: []});
  const [selectedSysno, setSelectedSysno] = useState<number>(0);
  const navigate = useNavigate();

  useEffect(() => {
    invoke('get_syscall_list').then((json) => {
      console.log(json);
      setSyscalls(old => json);
      setSelectedSysno(0);
    });
  }, []);

  function handleFinish() {
    invoke('set_syscall_list', {syscallList: syscalls}).then(() => {
      navigate("/export");
    }).catch((error) => {
      alert("Couldn't send syscall list due to internal error: " + error);
    });
  }

  return (
    <div style={{ height: '100%' }}>
      <div className='syscall-pane'>
        <div className='bordered-list syscall-list'>
          {syscalls.syscalls.map((item) => (
              selectedSysno === item.syscall.id ? (
                <div>
                  <label key={item.syscall.id}>
                    <input type="checkbox" style={{ marginRight: '8px' }} defaultChecked={item.enabled} onChange={() => item.enabled = !item.enabled}/>
                    <span className='selected' onClick={() => setSelectedSysno(item.syscall.id)}>{item.syscall.name}</span>
                  </label>
                </div>
              ) : (
                <div>
                  <label key={item.syscall.id}>
                    <input type="checkbox" style={{ marginRight: '8px' }} defaultChecked={item.enabled} onChange={() => item.enabled = !item.enabled}/>
                    <span className='selected' onClick={() => setSelectedSysno(item.syscall.id)}>{item.syscall.name}</span>
                  </label>
                </div>
              )
          ))}
        </div>
        <div className='description-pane'>
            <p>TEST</p>
        </div>
      </div>
      <div style={{ float: 'right' }}>
        <button onClick={() => navigate("/")}>Back</button>
        <button onClick={handleFinish}>Next</button>
      </div>
    </div>
  );
}

export default SyscallPage;