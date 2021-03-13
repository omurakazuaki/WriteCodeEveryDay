import React from 'react';
import { useDispatch, useSelector } from 'react-redux';
import { RootState } from '../../app/store';
import { TodoState, save, updateSelected } from './todoSlice';
import { Grid, TextField, Button } from '@material-ui/core';

export default function Detail() {
  const state = useSelector<RootState, TodoState>(state => state.todo);
  const dispatch = useDispatch();

  if (!state.selected) {
    return (
      <Grid container spacing={3}>
        <Grid item xs={12}>
          <p>No items selected</p>
        </Grid>
      </Grid>
    );
  }

  const newTodo = {...state.selected};

  return (
    <form>
      <Grid container spacing={3}>
        <Grid item xs={12}>
          <TextField fullWidth label="Title" value={newTodo.title} onChange={e => {
            newTodo.title = e.target.value;
            dispatch(updateSelected(newTodo));
          }}/>
        </Grid>
        <Grid item xs={12}>
          <TextField type="date" fullWidth label="Deadline" value={newTodo.deadline} onChange={e => {
            newTodo.deadline = e.target.value;
            dispatch(updateSelected(newTodo));
          }} />
        </Grid>
        <Grid item xs={12}>
          <Button variant="contained" color="primary" onClick={()=>dispatch(save(newTodo))}>
            Update
          </Button>
        </Grid>
      </Grid>
    </form>
  );
}
