export const Users = [
  {
    id: 1,
    name: "Taro Yamada",
    email: "taro@mail.com",
    posts: [
      {
        id: 1,
        title: "My favorite languages",
        published: true,
        link:
          "https://blog.com/@taroyamada/6e139b79e8d2",
        author: {id:1}
      },
      {
        id: 2,
        title: "GraphGL Sample code",
        published: true,
        link:
          "https://blog.com/@taroyamada/b55e12514c46",
        author: {id:1}
      }
     ]
  },
  {
    id: 2,
    name: "Hanako Sato",
    email: "hanako@mail.com",
    posts: [
      {
        id: 3,
        title: "Getting started with git",
        published: true,
        link:
          "https://blog.com/@hanakosato/b55e12514c46",
        author: {id:2}
      }
    ]
  }
];

