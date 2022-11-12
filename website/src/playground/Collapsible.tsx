import { useState } from "react";

interface Props {
  className?: string;
  heading: string | JSX.Element;
  children: JSX.Element;
}

export default function Collapsible(props: Props) {
  const [visible, setVisible] = useState(true);

  function onClick() {
    //setVisible(!visible);
  }

  let className = visible ? "" : "collapsed";
  
  if (props.className != null) {
    className += ` ${props.className}`;
  }

  return <div className={`collapsible-container ${className}`}>
    <h2 onClick={onClick} className={`${className}`}>{props.heading}</h2>
    {visible && <div className="collapsible-content">{props.children}</div>}
  </div>;
}
