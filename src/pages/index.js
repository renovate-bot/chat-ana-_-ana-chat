import Head from 'next/head'
import Image from 'next/image'
import { Inter } from '@next/font/google'
import { FormEvent, useEffect, useRef, useState } from 'react'
import Link from 'next/link'
import { useRouter } from 'next/router'

const inter = Inter({ subsets: ['latin'] })

export default function Home() {
  const [dom_v, dom_s] = useState(false);
  const [members_v, members_s] = useState(false);
  const {id} =  useRouter().query;
  // console.log(id)
  let message = useRef(null);
  useEffect( () => {
    document.addEventListener("mousemove", (e) => {
      dom_s(e.clientX <= 15)
      members_s(window.innerWidth - e.clientX <= 10)
    })
  })
  const sendMessage = (e) => {
    e?.preventDefault();

    if (message.current?.value){
      fetch("http://127.0.0.1:8000/chat/send", {
        method: "POST",
        headers: {
          "chatid": "a",
          "chat": {
            "_id": "0",
            "sender": "523",
            "content": "asdf",
            "date": new Date().getTime()
          }
        }
      }).then(e => { console.log(e) })

      // fetch("http://127.0.0.1:8000/user/create", {
      //   method: "POST",
      //   headers: {
      //     "name": "523",
      //     "email": "yhanbyeol6@gmail.com",
      //     "profile_image": "a",
      //   }
      // }).then(e => { console.log(e) })



      // fetch("http://127.0.0.1:8000/user/info", {
      //   method: "GET",
      //   headers: {
      //     "name": "523",
      //     "email": "yhanbyeol6@gmail.com",
      //     "profile_image": "a",
      //   }
      // }).then(e => { console.log(e) })

      //server info
      fetch("http://127.0.0.1:8000/server/info", {
        headers: {
          "name": "a"
        }
      }).then( e => { e.json().then(e => {
        console.log(e)
      }) })
      // fetch("http://127.0.0.1:8000/server/create", {
      //   method: "POST",
      //   headers: {
      //     "name": "a"
      //   }
      // }).then( e => { console.log(e) })

      message.current.value = "";
    } else { console.log("non") } 
  }
  return (
    <div id="main">
      <link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Material+Symbols+Rounded:opsz,wght,FILL,GRAD@48,400,0,0" />

      <nav className={String(dom_v)}>
        <ServerBtn id="logo" select={true}/>
        
        <hr/>
        <ServerBtn id="1"/>
        <ServerBtn id="2"/>
        <ServerBtn id="3"/>
        <ServerBtn id="4"/>
        <ServerBtn id="5"/>
      </nav>

      <main>
        <Msg name="5-23" msg="asdf"/>
      </main>
        
      <form id='messageSender' onSubmit={e => { sendMessage(e) }}>
          <input type="text" ref={message} placeholder='메세지를 입력해주세요' />
          <img src="/icon/send_w.svg" onClick={() => sendMessage()}/>
      </form>

      <aside className={String(members_v)}>
        <UserInfo id='1' name="eaaaaaaaaaaaaaaaa" url="/server/ea.png"/>
        <UserInfo id='1' name="eee" url="ea"/>
        <UserInfo id='1' name="s" url="ea"/>
        <UserInfo id='1' name="ddf" url="ea"/>
        <UserInfo id='1' name="5-23" url="ea"/>
      </aside>
    </div>
  )
}

function ServerBtn(props){
  if (props.id == "logo") {
    return (
      <Link href={`?id=${props.id}`} className={`serverBtn ${props.select}`}>
        <img src={`/logo.svg`}/>
      </Link>
    )
  }
  return (
    <Link href={`?id=${props.id}`} className={`serverBtn ${props.select}`}>
      <img src={`/server/${props.id}.png`}/>
    </Link>
  )
}

function UserInfo(props){
  return (
    <a className={`userInfo`}>
      <img src={`/user/${props.id}.png`}/> <b>{props.name}</b>
      <span></span>
    </a>
  )
}

function Msg(props){
  return (
    <section>
      <b>{props.name}</b>: {props.msg}</section>
  )
}