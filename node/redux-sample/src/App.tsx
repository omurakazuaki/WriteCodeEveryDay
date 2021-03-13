import React from 'react';
import './App.css';
import Paper from '@material-ui/core/Paper';
import Grid from '@material-ui/core/Grid';
import TodoList from './features/todo/List';
import Detail from './features/todo/detail';

function App() {
  return (
    <div>
      <Grid container>
        <Grid item xs={3}>
          <Paper><TodoList/></Paper>
        </Grid>
        <Grid item xs={9}>
          <Paper><Detail/></Paper>
        </Grid>
      </Grid>
    </div>
  );
}

export default App;
