import React from 'react';
import { View } from 'react-native';
import { ListItem, Avatar } from 'react-native-elements'
import { ParamListBase } from '@react-navigation/native';
import { StackNavigationProp } from '@react-navigation/stack';

const list = [
  {
    title: '新規TODO作成',
    subtitle: '空のTODO編集ページに遷移'
  },
  {
    title: 'TODO編集',
    subtitle: '編集ページに遷移'
  },
  {
    title: 'TODOデータを保存',
    subtitle: 'ローカルストレージにデータを保存する'
  },
  {
    title: '認証',
    subtitle: 'ログイン時に認証する'
  },
  {
    title: 'クローズ',
    subtitle: 'スワップしてクローズする'
  },
];

export default function TodoList({ navigation } : { navigation : StackNavigationProp<ParamListBase>}) {
  const handleClick = (item: any) => {
    return () => {
        navigation.push('Todo Detail', {
        item
      });
    }
  };
  return (
    <View>
      {
        list.map((l, i) => (
          <ListItem onPress={handleClick(l)} key={i} bottomDivider>
            <ListItem.Content>
              <ListItem.Title>{l.title}</ListItem.Title>
              <ListItem.Subtitle>{l.subtitle}</ListItem.Subtitle>
            </ListItem.Content>
          </ListItem>
        ))
      }
    </View>
  );
}
