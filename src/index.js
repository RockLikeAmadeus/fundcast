import React from "react";
import ReactDOM from "react-dom";
import { AccountListItem } from "./config-view/accounts/AccountList";
import { mySampleData } from "./sampleData";
ReactDOM.createRoot(document.getElementById("root")).render(
  <AccountListItem account={mySampleData} />
);
