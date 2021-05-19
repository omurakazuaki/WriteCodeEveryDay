import React from 'react';
import { NavigationContainer } from '@react-navigation/native';
import { createStackNavigator } from '@react-navigation/stack';
import { StyleSheet, View } from 'react-native';
import TodoList from './screens/TodoList';
import TodoDetail from './screens/TodoDetail';
import TodoEditor from './screens/TodoEditor';
import { Provider } from 'react-redux';
import { store } from './app/store';

const Stack = createStackNavigator();

export default function App() {
  return (
    <NavigationContainer>
      <Provider store={store}>
        <View style={styles.container}>
        <Stack.Navigator>
          <Stack.Screen name="Todo List" component={TodoList} />
          <Stack.Screen name="Todo Detail" component={TodoDetail} />
          <Stack.Screen name="Todo Editor" component={TodoEditor} />
        </Stack.Navigator>
        </View>
      </Provider>
    </NavigationContainer>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1
  },
});
