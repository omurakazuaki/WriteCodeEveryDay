import React from 'react';
import { useSelector, useDispatch } from 'react-redux';
import { RootState } from '../../app/store';
import { TodoState, select } from './todoSlice';
import {List, ListItem, ListItemText, ListItemIcon, Icon, Divider } from '@material-ui/core';
import TodoListItem from './ListItem';
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
        {state.list.map(i => (<TodoListItem todo={i} selected={state.selected?.id===i.id}/>))}
      </List>
    </div>
  );
}
