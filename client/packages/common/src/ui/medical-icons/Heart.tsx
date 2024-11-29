import React from 'react';
import SvgIcon, { SvgIconProps } from '@mui/material/SvgIcon';

export const HeartIcon = (props: SvgIconProps): JSX.Element => (
  <SvgIcon {...props} viewBox="0 0 60 60" fill="none">
    <path
      d="M37.6229 16.7759C35.1023 17.0642 33.6368 18.5365 32.3409 19.8358C31.5335 20.6466 30.7719 21.4116 29.8576 21.7644C29.5082 21.9001 29.1147 21.7254 28.9807 21.376C28.845 21.0266 29.0197 20.6331 29.3691 20.4991C30.0154 20.2497 30.6481 19.6136 31.3808 18.8792C32.7599 17.4934 34.4764 15.77 37.4687 15.4274C38.9292 15.2578 40.1148 15.2697 41.1597 15.4461L41.923 13.5073C39.9927 12.5201 37.601 12.7101 37.5755 12.7101C29.4185 13.3988 27.1201 21.0947 27.098 21.1724C27.0131 21.4692 26.7418 21.6643 26.4466 21.6643C26.3855 21.6643 26.3211 21.6558 26.26 21.6371C25.9004 21.5337 25.6918 21.1588 25.7953 20.7975C25.8071 20.7551 26.4381 18.6518 28.1547 16.4365L28.1513 11H25.0879V15.8343C25.0879 16.2091 24.7843 16.5128 24.4095 16.5128C24.0346 16.5128 23.731 16.2091 23.731 15.8343V13.4223C23.3714 12.7506 22.888 12.106 22.2926 11.4988L19.9247 13.8666C21.1714 15.2016 22.0043 16.5025 22.1654 16.7604C22.3452 16.9894 23.753 18.8298 24.6792 21.4149C24.8047 21.7677 24.6215 22.1561 24.2687 22.2834C24.1924 22.3105 24.1161 22.3241 24.0397 22.3241C23.7616 22.3241 23.5003 22.151 23.4019 21.8745C22.7777 20.1326 21.8991 18.7332 21.4225 18.0479C19.7737 19.03 14.5378 22.6225 14.0119 28.8255C14.9889 29.582 16.4562 30.3758 18.2932 30.3385C22.2826 30.2537 24.3993 24.4968 24.4197 24.4391C24.5469 24.0862 24.937 23.903 25.2882 24.0286C25.641 24.1558 25.8242 24.5425 25.6986 24.8953C25.6003 25.1684 23.2476 31.5903 18.322 31.6954C18.2711 31.6954 18.2202 31.6971 18.1693 31.6971C16.4799 31.6971 15.0754 31.1305 14 30.4639C14.3596 35.4762 18.9884 41.5776 23.312 45.2599C26.2957 47.8008 29.1199 49.2239 30.6852 48.9712C35.2768 48.2316 40.3199 38.6022 40.8779 31.6802C41.4224 24.9158 36.9732 22.1254 36.7833 22.0099C36.6136 21.9064 36.4949 21.7317 36.4644 21.5349C36.4338 21.3382 36.4898 21.1363 36.6204 20.9854C37.4295 20.0508 38.4048 19.5572 39.5243 19.5165C41.2375 19.4622 42.8455 20.5223 43.663 21.1804L45.227 19.2332C43.8904 17.9746 42.8285 17.2147 41.4309 16.8754C40.4097 16.6244 39.202 16.5939 37.628 16.7771L37.6229 16.7759ZM35.9708 33.224C35.8622 33.4836 35.6095 33.6396 35.3449 33.6396C35.2567 33.6396 35.1685 33.6226 35.0819 33.5853C33.6402 32.9781 32.7429 31.9162 32.2018 30.9901C31.8693 31.75 31.4266 32.4675 30.8381 33.0917C31.1705 33.8126 31.6828 35.3018 31.6438 37.4474C32.4104 37.7171 33.6996 38.4482 34.1253 40.3802C34.2067 40.7466 33.9743 41.1079 33.6096 41.1893C33.5605 41.1995 33.5113 41.2046 33.4638 41.2046C33.1517 41.2046 32.8718 40.9891 32.8023 40.6719C32.575 39.644 32.0051 39.1403 31.5437 38.8875C31.3792 40.1647 31.0179 41.6201 30.3428 43.2415C30.2342 43.5027 29.9815 43.6588 29.7169 43.6588C29.6304 43.6588 29.5422 43.6418 29.4557 43.6062C29.1097 43.462 28.9468 43.0651 29.091 42.7191C30.9314 38.3004 30.2478 35.2946 29.761 34.0193C26.6129 36.2786 25.97 39.0316 25.9632 39.0603C25.8937 39.3775 25.6121 39.5946 25.3 39.5946C25.2525 39.5946 25.205 39.5895 25.1575 39.5793C24.7912 39.5013 24.5588 39.1417 24.6368 38.777C24.6572 38.6786 25.0541 36.9264 26.7147 34.9775C25.9259 34.8367 24.7505 34.8079 23.5309 35.4372C23.4325 35.4881 23.3257 35.5118 23.2205 35.5118C22.9746 35.5118 22.7371 35.3778 22.6167 35.1437C22.4453 34.8113 22.576 34.4008 22.9084 34.2295C24.9795 33.1609 26.9522 33.5781 27.8035 33.8512C28.1868 33.5035 28.6143 33.1591 29.0926 32.8233C30.4174 31.8955 31.1128 30.4367 31.4622 28.9475C31.4639 28.9407 31.4656 28.9356 31.4656 28.9288C31.7964 27.5074 31.8133 26.0622 31.754 25.0224C31.7489 24.9325 31.7455 24.8698 31.7455 24.8392C31.7455 24.4644 32.0491 24.1608 32.424 24.1608C32.7988 24.1608 33.1007 24.4627 33.1024 24.8358L33.1075 24.9478C33.1686 26.0384 33.1499 27.5311 32.826 29.0424C32.982 29.5632 33.6826 31.5206 35.6128 32.3331C35.9589 32.4789 36.12 32.8759 35.9758 33.2219L35.9708 33.224Z"
      fill="url(#paint0_linear_247_41345)"
    />
    <defs>
      <linearGradient
        id="paint0_linear_247_41345"
        x1="29.6135"
        y1="11"
        x2="29.6135"
        y2="49"
        gradientUnits="userSpaceOnUse"
      >
        <stop stop-color="#E73735" />
        <stop offset="1" stop-color="#FC7E08" />
      </linearGradient>
    </defs>
  </SvgIcon>
);
