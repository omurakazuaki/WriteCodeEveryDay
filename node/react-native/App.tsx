import React from 'react';
import { NavigationContainer } from '@react-navigation/native';
import { createStackNavigator } from '@react-navigation/stack';
import { StyleSheet, View } from 'react-native';
import TodoList from './screens/TodoList';
import TodoDetail from './screens/TodoDetail';

const Stack = createStackNavigator();

export default function App() {
  return (
    <NavigationContainer>
      <View style={styles.container}>
      <Stack.Navigator>
        <Stack.Screen name="Todo List" component={TodoList} />
        <Stack.Screen name="Todo Detail" component={TodoDetail} />
      </Stack.Navigator>
      </View>
    </NavigationContainer>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1
  },
});
