import { BrowserRouter, Routes, Route } from 'react-router-dom';
import { invoke } from "@tauri-apps/api/tauri";
import { useState } from 'react';

interface LogEntry {
    id: number
    text: string;
}

type LogEntryArray = LogEntry[] | [];

function LogPage() {
  const [list, setList] = useState<LogEntryArray>([]);

  return (
    
    <ul>
        {list.map((item) => (
            <li key={item.id}>{item.text}</li>
        ))}
    </ul>
  );
}

export default LogPage;