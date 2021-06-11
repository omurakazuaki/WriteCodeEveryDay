import Layout from '../../components/layout'
import { getTagData, getAllTagIds } from '../../lib/posts'
import Head from 'next/head'
import Link from 'next/link'
import Date from '../../components/date'
import utilStyles from '../../styles/utils.module.css'


export default function Tag({ tagData }) {
  return (
    <Layout>
      <Head>
        <title>{tagData.id}</title>
      </Head>
      <section className={`${utilStyles.headingMd} ${utilStyles.padding1px}`}>
        <h2 className={utilStyles.headingLg}>Blog</h2>
        <ul className={utilStyles.list}>
          {tagData.posts.map(({ id, date, title }) => (
            <li className={utilStyles.listItem} key={id}>
              <Link href={`/posts/${id}`}>
                <a>{id}</a>
              </Link>
              <br />
              <small className={utilStyles.lightText}>
                <Date dateString={date} />
              </small>
            </li>
          ))}
        </ul>
      </section>
    </Layout>
  )
}

export async function getStaticPaths() {
  const paths = getAllTagIds()
  return {
    paths,
    fallback: false
  }
}

export async function getStaticProps({ params }) {
  const tagData = await getTagData(params.id)
  return {
    props: {
      tagData
    }
  }
}
