import Layout from '../../components/layout'
import { getAllPostIds, getPostData } from '../../lib/posts'
import Head from 'next/head'
import Link from 'next/link'
import Date from '../../components/date'
import utilStyles from '../../styles/utils.module.css'
import { MDXProvider } from "@mdx-js/react"
import unified from 'unified'
import parse from 'remark-parse'
import remark2react from 'remark-react'

export default function Post({ postData, content }) {
  const components = {
    h2: props => <h2 id={props.children} style={{paddingTop: 32, paddingBottom: 8, border: 0, borderBottom: 1, borderStyle: 'solid', borderColor: '#b0b0bb'}} {...props}/>,
    h3: props => <h3 id={props.children} style={{paddingTop: 16}} {...props}/>,
  };

  return (
    <Layout>
      <Head>
        <title>{postData.title}</title>
      </Head>
      <article>
        <h1 className={utilStyles.headingXl}>{postData.title}</h1>
        <div className={utilStyles.lightText}>
          <Date dateString={postData.date} />
        </div>
        <div className={utilStyles.lightText}>
          {
            postData.tags.map(tag => <Link key={tag} href={`/tags/${tag}`}><a style={{marginRight: 8}}>{tag}</a></Link>)
          }
        </div>
        <MDXProvider components={components}>
          {unified()
            .use(parse)
            .use(remark2react)
            .processSync(postData.markdown).result}
        </MDXProvider>
      </article>
    </Layout>
  )
}

export async function getStaticPaths() {
  const paths = getAllPostIds()
  return {
    paths,
    fallback: false
  }
}

export async function getStaticProps({ params }) {
  const postData = await getPostData(params.id)
  return {
    props: {
      postData
    }
  }
}
