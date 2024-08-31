import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { Button } from "./components/ui/button";
import { Input } from "./components/ui/input";

function App() {
  const [items, setItems] = useState<string[]>([]);
  const [newItem, setNewItem] = useState("");
  const [errMsg, setErrMsg] = useState<string>();

  function addItem() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    invoke<string[]>("add_item", { newItem })
      .then(setItems)
      .then(() => setNewItem(""))
      .catch((error) => setErrMsg(error.toString()));
  }

  function clearItems() {
    invoke("clear_items")
      .then(() => setItems([]))
      .then(() => setNewItem(""))
      .then(() => setErrMsg(""))
      .catch((error) => setErrMsg(error.toString()));
  }

  function getItems() {
    invoke<string[]>("get_items")
      .then(setItems)
      .catch((error) => setErrMsg(error.toString()));
  }

  return (
    <div className="flex flex-col items-center min-h-screen m-10 gap-3">
      <Input value={newItem} onChange={(e) => setNewItem(e.target.value)} />
      <div className="flex flex-row gap-2">
        <Button onClick={addItem} disabled={!newItem.length}>
          Add
        </Button>
        <Button onClick={getItems} className="bg-blue-700">
          Get
        </Button>
        <Button onClick={clearItems} className="bg-red-500">
          Clear
        </Button>
      </div>
      {errMsg ? (
        <p className="text-red-500">{errMsg}</p>
      ) : (
        <div className="justify-start">
          {items.map((item, index) => (
            <p key={index}>
              {"\u2022"}
              {item}
            </p>
          ))}
        </div>
      )}
    </div>
  );
}

export default App;
