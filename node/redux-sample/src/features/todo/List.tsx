import React from 'react';
import { useSelector, useDispatch } from 'react-redux';
import { RootState } from '../../app/store';
import { TodoState, select } from './todoSlice';
import List from '@material-ui/core/List';
import ListItem from '@material-ui/core/ListItem';
import ListItemText from '@material-ui/core/ListItemText';
import ListItemIcon from '@material-ui/core/ListItemIcon';
import Icon from '@material-ui/core/Icon';
import TodoListItem from './ListItem';
import Divider from '@material-ui/core/Divider';
export default function TodoList() {

  const state = useSelector<RootState, TodoState>(state => state.todo);
  const dispatch = useDispatch();

  return (
    <div>
      <List component="nav" aria-label="main mailbox folders">
        <ListItem button onClick={() => dispatch(select(null))}>
         <ListItemIcon>
            <Icon>add</Icon>
          </ListItemIcon>
          <ListItemText primary="New" />
        </ListItem>
      </List>
      <Divider />
      <List>
        {state.list.map(i => (<TodoListItem todo={i}/>))}
      </List>
    </div>
  );
}
