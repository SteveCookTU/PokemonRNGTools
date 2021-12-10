import React from 'react';
import ReactDOM from 'react-dom';
import { HashRouter, Routes, Route } from 'react-router-dom';
import { MainLayout } from './Layouts/Main';
import { SwSh } from './SwSh/SwSh';
import { Wild } from './BdSp/Wild/Wild';
import { Stationary } from './BdSp/Stationary/Stationary';
import { Underground } from './BdSp/Underground/Underground';

const app = document.getElementById('app');
ReactDOM.render(
  <HashRouter>
    <Routes>
      <Route path="/" element={<MainLayout />}>
        <Route path="swsh" element={<SwSh />} />
        <Route path="bdsp" element={<Wild />} />
        <Route path="bdsp/stationary" element={<Stationary />} />
        <Route path="bdsp/underground" element={<Underground />} />
      </Route>
    </Routes>
  </HashRouter>,
  app,
);
