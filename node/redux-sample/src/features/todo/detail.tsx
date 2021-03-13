import React from 'react';
import { useDispatch, useSelector } from 'react-redux';
import { RootState } from '../../app/store';
import { TodoState, update } from './todoSlice';
import TextField from '@material-ui/core/TextField';
import Button from '@material-ui/core/Button';

export default function Detail() {
  const state = useSelector<RootState, TodoState>(state => state.todo);
  const dispatch = useDispatch();

  return (
    <form>
      <div>
        <TextField fullWidth label="Title" value={state.selected?.title}/>
      </div>
      <div>
        <TextField fullWidth label="Deadline" value={state.selected?.deadline} />
      </div>
      <Button variant="contained" color="primary" onClick={()=>dispatch(update(state.selected))}>
        Update
      </Button>
    </form>
  );
}
