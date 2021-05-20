import React, { useEffect } from 'react';
import { StyleSheet, ScrollView, View } from 'react-native';
import { ListItem, FAB } from 'react-native-elements'
import { ParamListBase, useNavigation } from '@react-navigation/native';
import { StackNavigationProp } from '@react-navigation/stack';
import { useDispatch, useSelector } from 'react-redux';
import { load, select, TodoState, fetchAllTodo } from '../slices/todoSlice';
import { RootState } from '../app/store';
import { useCallback } from 'react';

export default function TodoList() {
  const navigation = useNavigation<StackNavigationProp<ParamListBase>>();
  const state = useSelector<RootState, TodoState>(state => state.todo);
  const dispatch = useDispatch();

  const handleClickItem = (id: number | null) => {
    return () => {
        dispatch(select(id));
        navigation.push('Todo Detail');
    }
  };

  const handleClickNew = () => {
    dispatch(select(null));
    navigation.push('Todo Editor');
  };

  useCallback(() => {
    dispatch(fetchAllTodo())
  }, [dispatch]);

  return (
    <View style={styles.container}>
      <ScrollView>
        {
          state.list.map((l, i) => (
            <ListItem onPress={handleClickItem(l.id)} key={i} bottomDivider>
              <ListItem.Content>
                <ListItem.Title>{l.title}</ListItem.Title>
                <ListItem.Subtitle>{l.subtitle}</ListItem.Subtitle>
              </ListItem.Content>
            </ListItem>
          ))
        }
      </ScrollView>
      <FAB
        onPress={handleClickNew}
        icon={{name:'add', color: "#fff"}}
        color="#aaa"
        placement="right"
        size="small"/>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    height: '100%'
  }
});
