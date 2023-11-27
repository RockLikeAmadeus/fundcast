import React from "react";
import { render } from "react-dom";
import ReactDOM from "react-dom/client";
import { act } from "react-dom/test-utils";

describe("AccountListView", () => {
  it("renders...", () => {
    render(<AccountListView />);
    // expect(document.querySelector("AccountList")).not.toBeNull();
  });
});
