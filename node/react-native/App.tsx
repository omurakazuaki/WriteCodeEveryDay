import React from 'react';
import { NavigationContainer } from '@react-navigation/native';
import { createStackNavigator } from '@react-navigation/stack';
import { SafeAreaView, StyleSheet, StatusBar } from 'react-native';
import TodoList from './screens/TodoList';
import TodoDetail from './screens/TodoDetail';
import TodoEditor from './screens/TodoEditor';
import { Provider } from 'react-redux';
import { store } from './app/store';

const Stack = createStackNavigator();

export default function App() {
  return (
    <SafeAreaView style={styles.container}>
      <NavigationContainer>
        <Provider store={store}>
            <Stack.Navigator>
              <Stack.Screen name="Todo List" component={TodoList} />
              <Stack.Screen name="Todo Detail" component={TodoDetail} />
              <Stack.Screen name="Todo Editor" component={TodoEditor} />
            </Stack.Navigator>
        </Provider>
      </NavigationContainer>
    </SafeAreaView>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    paddingTop: StatusBar.currentHeight
  },
});
