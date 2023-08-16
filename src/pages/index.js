import Head from 'next/head'
import Image from 'next/image'
import { Inter } from 'next/font/google'
import { FormEvent, useEffect, useRef, useState } from 'react'
import Link from 'next/link'
import { useRouter } from 'next/router'

const inter = Inter({ subsets: ['latin'] })

function backend_string(endpoint) {
  let uri = new URL(`http://${window.location.hostname}`);
  if (window.location.hostname == "localhost" || window.location.hostname == "127.0.0.1") {
    uri.port = "8000"
  } else {
    uri.protocol = "https"
  }
  uri = uri.toString()
  return `${uri}${endpoint}`
}

export default function Home() {
  const [dom_v, dom_s] = useState(false);
  const [members_v, members_s] = useState(false);
  const {id} = useRouter().query;
  const [msg_v, msg_s] = useState([])
  const [msg_arr, msg_arr_s] = useState([])
  let message = useRef(null);

  useEffect( () => {
    let f = async () => {
      document.addEventListener("mousemove", (e) => {
        dom_s(e.clientX <= 15)
        members_s(window.innerWidth - e.clientX <= 10)
      })
  
      if (new Date().getTime() % 4){
        // server info
        fetch(backend_string("server/info"), {
          headers: {
            "name": id
          }
        }).then(e => { e.json().then(e => {
          msg_s(e.message.map( async e => chatid(e) ))
          console.log(e.message.map( async e => chatid(e) ))
        })
      })
    }
  }
    f()
    
    })
      
  const sendMessage = (e) => {
    e?.preventDefault();

    if (message.current?.value){
      fetch(backend_string("chat/info"), {
        method: "POST",
        headers: {
          "sender": "524",
          "content": encodeURI(message.current.value),
          "servername": "a"
        }
      }).then(e => { /*console.log(e)*/ })

      // fetch("http://127.0.0.1:8000/user/create", {
      //   method: "POST",
      //   headers: {
      //     "name": "524",
      //     "email": "yhanbyeol6bg@gmail.com",
      //     "profile_image": "a",
      //   }
      // }).then(e => { console.log(e) })


      // fetch("http://127.0.0.1:8000/user/info", {
      //   method: "GET",
      //   headers: {
      //     "email": "yhanbyeol6bg@gmail.com",
      //   }
      // }).then(e => { console.log(e) })



      // fetch("http://127.0.0.1:8000/server/join", {
      //   method: "POST",
      //   headers: {
      //     "servername": "a",
      //     "username": "524",
      //   }
      // }).then(e => { console.log(e) })

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
        {/* 
        msg_v.map( (e) => 
        {console.log( chatid(msg_v[1]))}
        )
         */}
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
// function chatid(id) {
//   var a = {
//   content: null,
//   date: null,
//   sender: null
//   }
//   fetch("http://127.0.0.1:8000/chat/info", {
//     method: "GET",
//     headers: {
//       "chatid": id
//     }
//   }).then(e => e.json().then(e => {
//     a.content = e.content
//     a.date = e.date
//     a.sender = e.sender
//   }))
//   return a
// }

async function chatid(id) {
  const a = await fetch(backend_string("chat/info"), {
    method: "GET",
    headers: {
      "chatid": id
    }
  })
  return await a.json()
}