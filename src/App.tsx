import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { Button } from "./components/ui/button";
import { Input } from "./components/ui/input";

function App() {
  const [items, setItems] = useState<string[]>([]);
  const [newItem, setNewItem] = useState("");

  function addItem() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    invoke<string[]>("add_item", { newItem })
      .then((items) => setItems(items))
      .catch((error) => setItems([error.toString()]));
  }

  return (
    <div className="flex flex-col items-center justify-center min-h-screen">
      <div className="flex flex-row gap-1">
        <Input value={newItem} onChange={(e) => setNewItem(e.target.value)} />
        <Button onClick={addItem}>Add item</Button>
      </div>
      <p>{items}</p>
    </div>
  );
}

export default App;
