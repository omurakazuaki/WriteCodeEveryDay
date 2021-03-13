import React from 'react';
import { ListItem, ListItemText} from '@material-ui/core';
import { Todo, select } from './todoSlice';
import { useDispatch } from 'react-redux';

interface Props {
  todo: Todo,
  selected: boolean
};

export default function Item(props: Props) {

  const dispatch = useDispatch();
  return (
    <ListItem button selected={props.selected} onClick={() => dispatch(select(props.todo.id))}>
      <ListItemText primary={props.todo.title} secondary={props.todo.deadline}/>
    </ListItem>
  );
}
