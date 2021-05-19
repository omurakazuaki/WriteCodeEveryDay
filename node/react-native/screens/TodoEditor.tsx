import React from 'react';
import { Text } from 'react-native';
import { Card, Input, Button } from 'react-native-elements'
import { StackNavigationProp } from '@react-navigation/stack';
import { useNavigation, ParamListBase } from '@react-navigation/native';
import { useSelector, useDispatch } from 'react-redux';
import { RootState } from '../app/store';
import { TodoState, updateSelected, save } from '../slices/todoSlice';

export default function TodoEditor() {
  const navigation = useNavigation<StackNavigationProp<ParamListBase>>();
  const state = useSelector<RootState, TodoState>(state => state.todo);
  const newTodo = {...state.selected};

  const dispatch = useDispatch();
  return (
    <Card>
      <Card.Title><Text>New Todo</Text></Card.Title>
      <Card.Divider/>
      <Input
        label="Title"
        value={newTodo.title}
        onChangeText={value=>{
          newTodo.title = value;
          dispatch(updateSelected(newTodo));
        }}
      />
      <Input
        label="Description"
        value={newTodo.subtitle}
        onChangeText={value=>{
          newTodo.subtitle = value;
          dispatch(updateSelected(newTodo));
        }}
      />
      <Button
        title="SAVE"
        onPress={()=>{
          dispatch(save(newTodo));
          navigation.popToTop();
        }}
       />
    </Card>
  );
}
