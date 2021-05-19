import { ParamListBase, useNavigation } from '@react-navigation/native';
import { StackNavigationProp } from '@react-navigation/stack';
import React from 'react';
import { StyleSheet, View, Text } from 'react-native';
import { Card, FAB } from 'react-native-elements'
import { useSelector } from 'react-redux';
import { RootState } from '../app/store';
import { TodoState } from '../slices/todoSlice';

export default function TodoEditor() {
  const navigation = useNavigation<StackNavigationProp<ParamListBase>>();
  const state = useSelector<RootState, TodoState>(state => state.todo);

  const item = {...state.selected};

  const handleClickNew = () => {
    navigation.push('Todo Editor');
  };

  return (
    <View style={styles.container}>
      <Card>
        <Card.Title>{item.title}</Card.Title>
        <Card.Divider/>
        <Text>{item.subtitle}</Text>
      </Card>
      <FAB
        onPress={handleClickNew}
        icon={{name:'edit', color: "#fff"}}
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
