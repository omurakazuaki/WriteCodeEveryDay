import { graphql } from 'gatsby';
import * as React from "react"
import { Link } from "gatsby"
import { StaticImage } from "gatsby-plugin-image"

import Layout from "../components/layout"
import Seo from "../components/seo"

const IndexPage = props => (
  <Layout>
    <Seo title="Home" />
    <h1>Hi people</h1>
    <p>Welcome to your new Gatsby site.</p>
    <p>Now go build something great.</p>
    <StaticImage
      src="../images/gatsby-astronaut.png"
      width={300}
      quality={95}
      formats={["auto", "webp", "avif"]}
      alt="A Gatsby astronaut"
      style={{ marginBottom: `1.45rem` }}
    />
    <ul>
      {
        props.data.allStrapiPost.edges.map(({node}) => {
          return <li><Link to={`/post/${node.slug}`}>{node.title}</Link> {node.created_at}</li>;
        })
      }
    </ul>
    <p>
      <Link to="/page-2/">Go to page 2</Link> <br />
      <Link to="/using-typescript/">Go to "Using TypeScript"</Link>
    </p>
  </Layout>
)

export const pageQuery =
  graphql`{
    allStrapiPost {
      edges {
        node {
          slug
          title
          tags{name}
          created_at
        }
      }
    }
  }`;
export default IndexPage
