import { BrowserRouter, Routes, Route, useNavigate } from 'react-router-dom';
import { listen } from '@tauri-apps/api/event';
import LogPage from "./pages/LogPage";
import SyscallPage from "./pages/SyscallPage";
import ExportPage from './pages/ExportPage';
import "./App.css";
import LoadPage from './pages/LoadPage';
import { useEffect } from 'react';
import NavigateListener from './components/NavigateListener';

function App() {
  return (
    <div className='container'>
      <div className='logPanel'>
        <h1>Log</h1>
        <LogPage />
      </div>

      <BrowserRouter>
        <NavigateListener>
          <Routes>
            <Route path="/" element={<LoadPage />} />
            <Route path="/syscalls" element={<SyscallPage />}/>
            <Route path="/export" element={<ExportPage />} />
          </Routes>
        </NavigateListener>
      </BrowserRouter>
    </div>
  );
}

export default App;
