import React from 'react';
import './App.css';
import TodoList from './features/todo/List';
import Detail from './features/todo/Detail';
import { Grid, Container, AppBar, Toolbar, Typography } from '@material-ui/core';

function App() {
  return (
    <Container>
      <Grid container spacing={3}>
        <Grid item xs={12}>
          <AppBar position="static">
            <Toolbar>
              <Typography variant="h6">
                TODO List
              </Typography>
            </Toolbar>
          </AppBar>
        </Grid>
        <Grid item xs={3}>
          <TodoList/>
        </Grid>
        <Grid item xs={9}>
          <Detail/>
        </Grid>
      </Grid>
    </Container>
  );
}

export default App;
