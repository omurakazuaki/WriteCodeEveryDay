import React from 'react';
import { Text } from 'react-native';
import { Card } from 'react-native-elements'

export default function TodoDetail({ route } : { route: any }) {
  const item = route.params.item;
  return (
    <Card>
      <Card.Title>{item.title}</Card.Title>
      <Card.Divider/>
      <Text>{item.subtitle}</Text>
    </Card>
  );
}
