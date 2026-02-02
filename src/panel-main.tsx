import React from "react";
import ReactDOM from "react-dom/client";
import { ControlPanel } from "./components/panel/ControlPanel";
import "./index.css";

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <ControlPanel />
  </React.StrictMode>
);
