import React from 'react';
import ListItem from '@material-ui/core/ListItem';
import ListItemText from '@material-ui/core/ListItemText';
import { Todo, select } from './todoSlice';
import { useDispatch } from 'react-redux';

interface Props {
  todo: Todo
};

export default function Item(props: Props) {

  const dispatch = useDispatch();
  return (
    <ListItem button onClick={() => dispatch(select(props.todo.id))}>
      <ListItemText primary={props.todo.title} secondary={props.todo.deadline}/>
    </ListItem>
  );
}
