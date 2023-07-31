import Head from 'next/head'
import Image from 'next/image'
import { Inter } from '@next/font/google'
import styles from '@/styles/Home.module.css'
import { useEffect, useState } from 'react'

const inter = Inter({ subsets: ['latin'] })

export default function Home() {
  const [dom_v, dom_s] = useState<boolean>(false);
  useEffect( () => {
    document.addEventListener("mousemove", (e) => {
      dom_s(e.offsetX <= 15)
    })
  })
  return (
    <div id="main">
      <nav className={String(dom_v)}>
        <ServerBtn url="logo.svg" select={true}/>
        
        <hr/>
        <ServerBtn url="server/ea.png"/>
        <ServerBtn url="server/ea.png"/>
        <ServerBtn url="server/ea.png"/>
        <ServerBtn url="server/ea.png"/>
        <ServerBtn url="server/ea.png"/>
      </nav>
      {/* <main>
        <section></section>
        <input type="text" />
      </main> */}
    </div>
  )
}

function ServerBtn(props: {url: string, select?: boolean}){
  console.log(props.select)
  return (
    <a className={`serverBtn ${props.select}`}>
      <img src={props.url}/>
    </a>
  )
}