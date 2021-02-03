import Link from 'next/link'
import { promisified } from "tauri/api/tauri"
import Layout from '../../components/layout'
export default function FirstPost() {
	var name, date_time, grandparent, parent, id;
promisified({ cmd: 'myCustomCommand', argument: 'something' })
	promisified({ cmd: 'spitTodos', name: "dis be do the nammee", date_time: 13, grandparent: "way back type shit", parent: "subTypy boiii", id: "67tbFdF"}).then(response => {
		[name, date_time, grandparent, parent, id] = [response.name, response.date_time, response.grandparent, response.parent, response.id]
	}).catch(error => {console.log("wellll fucjk ", error )})
  return (
    <Layout>
      <h1>First Post</h1>
      <h2>
        <Link href="/">
          <a>Back to home</a>
        </Link>
      </h2>
    </Layout>
  )
}

