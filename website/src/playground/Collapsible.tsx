import { useState } from "react";

interface Props {
  heading: JSX.Element;
  children: JSX.Element;
}

export default function Collapsible(props: Props) {
  const [visible, setVisible] = useState(true);

  function onClick() {
    setVisible(!visible);
  }

  const className = visible ? "" : "collapsed";

  return <div className={`collapsible-container ${className}`}>
    <h2 onClick={onClick} className={`collapsible ${className}`}>{props.heading}</h2>
    {visible && <div className="collapsible-content">{props.children}</div>}
  </div>;
}
