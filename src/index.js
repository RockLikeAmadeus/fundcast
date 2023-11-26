import React from "react";
import ReactDOM from "react-dom";
import { MyRootComponent } from "./MyRootComponent";
import { mySampleData } from "./sampleData";
ReactDOM.createRoot(document.getElementById("root")).render(
  <MyRootComponent data={mySampleData} />
);
