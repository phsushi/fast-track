import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { ping } from "./lib/tauri";


const result = await ping();

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <div className="container" style={{ width: "100%", height: "100%", alignItems: "center", justifyContent: "center" }}>
      result: {result}
    </div>
  );
}

export default App;
