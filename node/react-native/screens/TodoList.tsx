import React from 'react';
import { StyleSheet, View } from 'react-native';
import { ListItem, FAB } from 'react-native-elements'
import { ParamListBase, useNavigation } from '@react-navigation/native';
import { StackNavigationProp } from '@react-navigation/stack';
import { useDispatch, useSelector } from 'react-redux';
import { select, TodoState } from '../slices/todoSlice';
import { RootState } from '../app/store';

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

  return (
    <View style={styles.container}>
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
  },
});
