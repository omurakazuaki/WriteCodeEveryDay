import {Users} from '../data/blog';

let postId = 3;

export const blogResolvers = {
  users: async (_) => {
    return Users;
  },
  user: async ({ id }, _) => {
    return Users.find(user => user.id == id)
  },
  posts: async ({ authorId }) => {
    const posts = Users.reduce((acc, u) => acc.concat(u.posts), []);
    if (authorId) {
      return posts.filter(p => p.author === authorId);
    } else {
      return posts;
    }
  },
  registerPost({authorId, title, published}) {
    const user = Users.find(user => user.id == authorId)
    const post = {
      id: postId++,
      title: title,
      published: !!published,
      link: `https://blog.com/@${user.name.replace(' ', '').toLocaleLowerCase()}/dummy`,
      author: {id: authorId}
    };
    user.posts.push(post);
    return post;
  }
};
