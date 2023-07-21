import { App } from "./App";

import { StrictMode } from "react";

import { createRoot } from "react-dom/client";

import React from "react";
import { BrowserRouter } from "react-router-dom";
import { CssBaseline } from "@mui/material";
import { ApolloClient, InMemoryCache, ApolloProvider } from "@apollo/client";

const client = new ApolloClient({
  uri: "https://flyby-router-demo.herokuapp.com/",
  cache: new InMemoryCache(),
});

const container = document.getElementById("root") as Element;

const root = createRoot(container);
root.render(
  <StrictMode>
    <BrowserRouter>
      <CssBaseline>
        <ApolloProvider client={client}>
          <App />
        </ApolloProvider>
      </CssBaseline>
    </BrowserRouter>
  </StrictMode>
);
