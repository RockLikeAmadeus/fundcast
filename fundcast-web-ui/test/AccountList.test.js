import React from "react";
import ReactDOM from "react-dom/client";
import { act } from "react-dom/test-utils";
import { AccountListItem } from "../src/config-view/accounts/AccountList";

describe("AccountListItem", () => {
  //#region setup
  let container;
  const account = {
    accountName: "Chase Checking",
  };
  beforeEach(() => {
    container = document.createElement("div");
  });
  //#endregion

  const render = (component) => {
    document.body.replaceChildren(container);
    act(() => ReactDOM.createRoot(container).render(component));
  };

  it("renders the account name", () => {
    render(<AccountListItem account={account} />);
    expect(document.body.textContent).toContain("Chase Checking");
  });
});

// describe("AccountList", () => {
//   it("renders...", () => {
//     render(<AccountListView />);
//     // expect(document.querySelector("AccountList")).not.toBeNull();
//   });
// });
