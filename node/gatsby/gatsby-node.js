const path = require(`path`)

exports.createPages = async ({ graphql, actions }) => {
  const { createPage } = actions;
  const posts = await graphql(
    `{
        allStrapiPost {
          edges {
            node {
              slug
              title
              content
              tags{name}
              created_at
            }
          }
        }
      }`
    );
  console.log(`----------------------------------------------`);
  posts.data.allStrapiPost.edges.forEach(({node}) => {
    createPage({
      path: `post/${node.slug}`,
      component: path.resolve('src/templates/post.tsx'),
      context: {
        data: node
      }
    });
    console.log(node);
  });
};
