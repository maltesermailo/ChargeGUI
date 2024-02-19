import { useEffect } from 'react';
import { listen } from '@tauri-apps/api/event';
import { BrowserRouter, Routes, Route, useNavigate } from 'react-router-dom';

function NavigateListener({children}) {
    const navigate  = useNavigate();
    useEffect(() => {
      const listener = listen<string>('navigate', (event) => {
        navigate(event.payload);
      });
  
      navigate("/");
  
      return () => {
        listener.then((unlistenFn) => unlistenFn());
      }
    }, []);
  return (
    <div className='contentPanel'>
        {children}
    </div>
  );
}

export default NavigateListener;