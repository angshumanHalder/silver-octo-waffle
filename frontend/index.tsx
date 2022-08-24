import React from "react";
import { createRoot } from "react-dom/client";
import App from "./components/App/App";
import { initContract } from "./near-api";
import { ChakraProvider, ColorModeScript } from "@chakra-ui/react";
import theme from "./theme";

const reactRoot = createRoot(document.querySelector("#root") as HTMLElement);
initContract()
  .then(() => {
    reactRoot.render(
      <>
        <ColorModeScript initialColorMode="dark" />
        <ChakraProvider theme={theme}>
          <App />
        </ChakraProvider>
      </>
    );
  })
  .catch((e) => {
    reactRoot.render(
      <div style={{ color: "red" }}>
        Error: <code>{e.message}</code>
      </div>
    );
    console.error(e);
  });
