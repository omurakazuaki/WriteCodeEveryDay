import express from 'express';
import { graphqlHTTP } from 'express-graphql';
import { loadSchema } from './schema/loader';
import { blogResolvers } from './resolver/blog';
import logger from 'morgan';

const app = express();
app.use(logger('default'));
app.use(
  '/blog',
  graphqlHTTP({
    schema: loadSchema('blog'),
    rootValue: blogResolvers,
    graphiql: true
  })
);

app.listen(3000);

console.log(`🚀 Server ready at http://localhost:3000/blog`);
