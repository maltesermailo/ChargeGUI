import { BrowserRouter, Routes, Route } from 'react-router-dom';
import LogPage from "./pages/LogPage";
import SyscallPage from "./pages/SyscallPage";
import ExportPage from './pages/ExportPage';
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  return (
    <div className='container'>
      <div className='logPanel'>
        <LogPage />
      </div>

      <BrowserRouter>
        <Routes>
          <Route path="/" element={<SyscallPage />} />
          <Route path="/export" element={<ExportPage />} />
        </Routes>
      </BrowserRouter>
    </div>
  );
}

export default App;
