import React from "react";
import ReactDOM from "react-dom/client";
import { act } from "react-dom/test-utils";
import { AccountListItem } from "../src/config-view/accounts/AccountList";

describe("AccountListItem", () => {
  let container;
  beforeEach(() => {
    container = document.createElement("div");
  });

  const render = (component) => {
    document.body.replaceChildren(container);
    act(() => ReactDOM.createRoot(container).render(component));
  };

  it("renders the account name", () => {
    const account = {
      accountName: "Chase Checking",
    };
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
