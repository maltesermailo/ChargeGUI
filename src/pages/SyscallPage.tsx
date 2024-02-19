import { BrowserRouter, Routes, Route } from 'react-router-dom';
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

  useEffect(() => {
    invoke('get_syscall_list').then((json) => {
      console.log(json);
      setSyscalls(old => json);
      setSelectedSysno(0);
    });
  }, []);

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
        <button>Back</button>
        <button>Next</button>
      </div>
    </div>
  );
}

export default SyscallPage;