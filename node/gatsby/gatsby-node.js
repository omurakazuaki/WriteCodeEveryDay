const path = require(`path`)

exports.createPages = async ({ graphql, actions }) => {
  const { createPage } = actions;
  const posts = await graphql(
    `{
        allStrapiPost {
          edges {
            node {
              slug
            }
          }
        }
      }`
    );
  posts.data.allStrapiPost.edges.forEach(({node}) => {
    createPage({
      path: `post/${node.slug}`,
      component: path.resolve('src/templates/post.tsx'),
      context: {
        slug: node.slug
      }
    });
  });

  const tags = await graphql(
    `{
        allStrapiTag {
          edges {
            node {
              name
            }
          }
        }
      }`
    );
  tags.data.allStrapiTag.edges.forEach(({node}) => {
    createPage({
      path: `tag/${node.name}`,
      component: path.resolve('src/templates/tag.tsx'),
      context: {
        name: node.name
      }
    });
  });
};
