import React from 'react';
import SvgIcon, { SvgIconProps } from '@mui/material/SvgIcon';
import { GradientStops } from './_gradient';

export const NutritionIcon = (props: SvgIconProps): JSX.Element => (
  <SvgIcon {...props} viewBox="0 0 60 60" fill="none">
    <path
      d="M36.2924 23.4852C36.4703 23.5765 36.6771 23.5941 36.8678 23.5339C36.9589 23.5013 39.158 22.7713 40.1667 20.886C41.1754 19.0008 40.5443 16.7627 40.5205 16.6682C40.4932 16.5734 40.4471 16.4852 40.3849 16.4087C40.3227 16.3323 40.2456 16.2692 40.1584 16.2233C40.0713 16.1756 39.9752 16.1464 39.8763 16.1376C39.7773 16.1288 39.6776 16.1405 39.5834 16.1721C39.486 16.1999 37.2869 16.9298 36.2841 18.82C36.0027 19.3616 35.8244 19.9508 35.7583 20.5576C35.6658 21.3865 35.7243 22.2254 35.9308 23.0335C35.9582 23.1289 36.0042 23.218 36.0663 23.2956C36.1283 23.3731 36.2052 23.4376 36.2924 23.4852ZM47.3233 27.6027C46.5759 25.8192 44.5493 23.9174 42.4554 23.8253C41.6129 23.8235 40.7765 23.9675 39.983 24.2507L39.6044 24.3623C39.3505 24.4339 39.1108 24.5085 38.8764 24.5825C38.0139 24.8593 37.3907 25.064 36.6144 24.8388C36.1603 24.6955 35.7246 24.4992 35.3163 24.2541C35.1005 24.1362 34.8753 24.0183 34.6405 23.9052C33.9941 23.5911 33.2888 23.4166 32.5705 23.3931L32.168 23.3775H32.1539C31.5664 23.3675 30.9812 23.4522 30.4207 23.6284C30.616 23.939 30.7297 24.294 30.7512 24.6603C30.7727 25.0266 30.7012 25.3924 30.5435 25.7237C30.1374 26.5442 29.5856 27.2841 28.915 27.9072C29.105 27.9667 29.2595 28.0222 29.3623 28.0627L29.3925 28.0744L29.4217 28.087C29.693 28.2029 29.9382 28.3724 30.1425 28.5852C30.3467 28.7981 30.506 29.0499 30.6106 29.3258C30.7152 29.6016 30.7631 29.8957 30.7514 30.1905C30.7397 30.4853 30.6687 30.7747 30.5425 31.0414C30.1365 31.8619 29.5847 32.6018 28.914 33.2249C29.104 33.2843 29.2585 33.3399 29.3613 33.3803L29.3915 33.392L29.4208 33.4047C29.6921 33.5206 29.9372 33.69 30.1415 33.9029C30.3458 34.1157 30.505 34.3676 30.6096 34.6434C30.7143 34.9193 30.7622 35.2134 30.7505 35.5082C30.7388 35.803 30.6677 36.0924 30.5415 36.359C29.9891 37.4832 29.1578 38.4469 28.127 39.1585C28.7313 40.3789 29.4657 41.5306 30.3174 42.5933C30.9255 43.3412 31.7641 44.2227 32.9092 44.423C33.0769 44.4533 33.247 44.4686 33.4174 44.4688C34.1302 44.4296 34.8285 44.2524 35.4737 43.9469C35.9656 43.7031 36.5019 43.5618 37.0501 43.5318C37.429 43.5701 37.8006 43.6619 38.1538 43.8046C38.5866 43.974 39.0389 44.0886 39.5001 44.1457C40.7987 44.2788 42.1314 43.6687 43.4768 42.3428C45.8622 39.8704 47.3864 36.6945 47.8228 33.2868C48.1739 31.3818 48.0012 29.4172 47.3233 27.6027ZM23.1802 46.3019C22.8225 46.014 21.7598 45.016 21.5556 44.1945C21.366 42.9476 21.3084 41.6843 21.3836 40.4254V23.3219C21.3862 23.2167 21.3678 23.1119 21.3293 23.0139C21.2908 22.9159 21.2331 22.8265 21.1595 22.7511C21.086 22.6757 20.9981 22.6158 20.9011 22.5749C20.804 22.534 20.6998 22.5129 20.5945 22.5129C20.4892 22.5129 20.3849 22.534 20.2879 22.5749C20.1908 22.6158 20.1029 22.6757 20.0294 22.7511C19.9559 22.8265 19.8981 22.9159 19.8597 23.0139C19.8212 23.1119 19.8027 23.2167 19.8053 23.3219V40.3879C19.8009 40.5043 19.6801 43.2102 20.0236 44.575C20.3949 46.0559 22.0103 47.3881 22.1959 47.5318C22.2767 47.5965 22.3694 47.6446 22.4687 47.6734C22.568 47.7022 22.6721 47.7111 22.7749 47.6997C22.8777 47.6883 22.9772 47.6568 23.0679 47.6069C23.1585 47.557 23.2384 47.4898 23.303 47.409C23.3676 47.3283 23.4157 47.2356 23.4445 47.1362C23.4733 47.0369 23.4823 46.9329 23.4709 46.8301C23.4595 46.7273 23.4279 46.6277 23.3781 46.5371C23.3282 46.4465 23.261 46.3666 23.1802 46.3019ZM16.0338 34.7106C14.8405 34.3306 13.5524 34.3755 12.3885 34.8378C12.3101 34.8707 12.2393 34.9192 12.1802 34.9802C12.121 35.0413 12.0749 35.1136 12.0445 35.193C12.0127 35.2724 11.9977 35.3576 12.0003 35.4431C12.0029 35.5286 12.0231 35.6126 12.0596 35.69C12.6012 36.8195 13.5221 37.7232 14.6616 38.2434C15.1537 38.4291 15.676 38.5216 16.2019 38.5162C16.9207 38.5109 17.6326 38.3754 18.303 38.1162C18.3815 38.0828 18.4526 38.0342 18.5122 37.9733C18.5719 37.9123 18.6188 37.8401 18.6505 37.7609C18.7111 37.5998 18.7058 37.4212 18.6359 37.2639C18.0892 36.138 17.1698 35.2359 16.0338 34.7106ZM29.1513 35.193C29.1209 35.1136 29.0747 35.0413 29.0156 34.9802C28.9565 34.9192 28.8856 34.8707 28.8073 34.8378C27.6435 34.3755 26.3556 34.3306 25.1624 34.7106C24.0263 35.2357 23.1069 36.1379 22.5604 37.2639C22.5244 37.3415 22.5045 37.4255 22.5018 37.5109C22.4991 37.5964 22.5138 37.6815 22.5449 37.7611C22.576 37.8407 22.6229 37.9132 22.6827 37.9742C22.7426 38.0353 22.8142 38.0835 22.8932 38.1162C23.5637 38.3754 24.2755 38.5109 24.9943 38.5162C25.5202 38.5216 26.0426 38.4291 26.5346 38.2434C27.6741 37.7232 28.5949 36.8196 29.1362 35.69C29.1728 35.6127 29.193 35.5286 29.1956 35.4431C29.1982 35.3576 29.1831 35.2724 29.1513 35.193ZM16.0338 29.3944C14.8405 29.0141 13.5523 29.0591 12.3885 29.5216C12.3101 29.5545 12.2393 29.603 12.1802 29.664C12.121 29.7251 12.0749 29.7974 12.0445 29.8768C12.0127 29.9562 11.9977 30.0414 12.0003 30.1269C12.0029 30.2124 12.0231 30.2964 12.0596 30.3738C12.6012 31.5033 13.5221 32.407 14.6616 32.9272C15.1537 33.1127 15.676 33.2052 16.2019 33.2C16.9207 33.1947 17.6326 33.0592 18.303 32.8C18.3815 32.7666 18.4526 32.718 18.5122 32.6571C18.5719 32.5961 18.6188 32.5239 18.6505 32.4447C18.7111 32.2836 18.7058 32.105 18.6359 31.9477C18.0894 30.8213 17.1701 29.9186 16.0338 29.3929V29.3944ZM27.0385 32.685C27.0302 32.6894 27.0204 32.6928 27.0122 32.6976C27.0204 32.6947 27.0302 32.6894 27.0385 32.685ZM29.1362 30.3724C29.1728 30.295 29.193 30.211 29.1956 30.1254C29.1982 30.0399 29.1831 29.9548 29.1513 29.8753C29.1209 29.796 29.0747 29.7236 29.0156 29.6626C28.9565 29.6015 28.8856 29.5531 28.8073 29.5201C27.6436 29.0576 26.3556 29.0127 25.1624 29.3929C24.0263 29.9181 23.1069 30.8202 22.5604 31.9463C22.5244 32.0238 22.5045 32.1078 22.5018 32.1933C22.4991 32.2787 22.5138 32.3638 22.5449 32.4434C22.576 32.5231 22.6229 32.5956 22.6827 32.6566C22.7426 32.7176 22.8142 32.7659 22.8932 32.7985C23.5637 33.0577 24.2755 33.1933 24.9943 33.1986C25.5202 33.2037 26.0425 33.1112 26.5346 32.9257C26.6988 32.8606 26.8583 32.7844 27.0122 32.6976C27.9282 32.1419 28.6654 31.3348 29.1362 30.3724ZM20.563 20.0874C20.6483 20.0866 20.7326 20.069 20.811 20.0356C20.8895 20.0021 20.9605 19.9535 21.0201 19.8925C21.5031 19.3602 21.8858 18.7449 22.1496 18.0764C22.3443 17.5878 22.4465 17.0674 22.4512 16.5415C22.3776 15.291 21.8675 14.1059 21.0099 13.1929C20.9505 13.1319 20.8795 13.0833 20.801 13.0502C20.7225 13.0171 20.6382 13 20.553 13C20.4679 13 20.3836 13.0171 20.3051 13.0502C20.2266 13.0833 20.1556 13.1319 20.0962 13.1929C19.2453 14.1112 18.7388 15.2958 18.6627 16.5454C18.742 17.7946 19.2515 18.9778 20.1045 19.8939C20.226 20.0162 20.3907 20.0857 20.563 20.0874ZM16.0357 24.0753C14.8425 23.695 13.5543 23.7399 12.3904 24.2025C12.3121 24.2354 12.2412 24.2839 12.1821 24.3449C12.123 24.4059 12.0768 24.4783 12.0464 24.5577C12.0147 24.6371 11.9996 24.7223 12.0022 24.8078C12.0048 24.8933 12.025 24.9773 12.0615 25.0547C12.6031 26.1842 13.524 27.0878 14.6636 27.608C15.1557 27.7935 15.678 27.8861 16.2039 27.8809C16.9227 27.8756 17.6345 27.7401 18.305 27.4809C18.3835 27.4475 18.4546 27.3989 18.5142 27.3379C18.5738 27.277 18.6208 27.2048 18.6524 27.1256C18.713 26.9645 18.7078 26.7859 18.6378 26.6286C18.0906 25.5024 17.1706 24.6002 16.0338 24.0753H16.0357ZM17.1336 18.9389C16.0509 18.3083 14.7832 18.0724 13.5462 18.2713C13.4625 18.2869 13.3827 18.3192 13.3117 18.3661C13.2406 18.4131 13.1797 18.4738 13.1325 18.5447C13.0853 18.6156 13.0528 18.6953 13.0369 18.779C13.021 18.8627 13.022 18.9487 13.04 19.032C13.323 20.2522 14.0254 21.3343 15.0246 22.0896C15.465 22.3776 15.9551 22.5812 16.4699 22.6899C17.1726 22.8409 17.8969 22.8634 18.6076 22.7562C18.6918 22.7416 18.7722 22.71 18.8439 22.6634C18.9156 22.6169 18.9772 22.5563 19.0249 22.4853C19.0725 22.4143 19.1053 22.3344 19.1213 22.2504C19.1373 22.1664 19.1361 22.0801 19.1178 21.9965C18.8292 20.7788 18.1281 19.6985 17.1336 18.9389ZM22.2748 22.4848C22.323 22.5553 22.3846 22.6155 22.4561 22.6621C22.5276 22.7087 22.6076 22.7407 22.6915 22.7562C23.4022 22.8634 24.1264 22.8409 24.8292 22.6899C25.3437 22.5809 25.8334 22.3772 26.2734 22.0891C27.2727 21.3338 27.9751 20.2517 28.2581 19.0315C28.276 18.9482 28.277 18.8622 28.261 18.7786C28.2451 18.695 28.2126 18.6154 28.1654 18.5445C28.1182 18.4737 28.0573 18.413 27.9863 18.3661C27.9152 18.3192 27.8355 18.2869 27.7518 18.2713C26.5163 18.0734 25.2503 18.3093 24.1689 18.9389C23.174 19.6983 22.4725 20.7787 22.1837 21.9965C22.1486 22.1649 22.1814 22.3404 22.2748 22.4848ZM27.0385 27.3673C27.0302 27.3717 27.0204 27.3751 27.0122 27.38L27.0385 27.3673ZM25.1624 24.0753C24.0263 24.6004 23.1069 25.5026 22.5604 26.6286C22.5244 26.7061 22.5045 26.7902 22.5018 26.8756C22.4991 26.961 22.5138 27.0461 22.5449 27.1258C22.576 27.2054 22.6229 27.2779 22.6827 27.3389C22.7426 27.4 22.8142 27.4482 22.8932 27.4809C23.5637 27.7401 24.2755 27.8756 24.9943 27.8809C25.5202 27.886 26.0425 27.7935 26.5346 27.608C26.6988 27.5433 26.8587 27.4671 27.0122 27.38C27.9282 26.8243 28.6649 26.0172 29.1357 25.0547C29.1723 24.9774 29.1926 24.8933 29.1952 24.8078C29.1978 24.7222 29.1827 24.6371 29.1508 24.5577C29.1204 24.4783 29.0742 24.4059 29.0151 24.3449C28.956 24.2839 28.8851 24.2354 28.8068 24.2025C27.6432 23.7401 26.3554 23.6951 25.1624 24.0753Z"
      fill="url(#paint0_linear_247_41620)"
    />
    <defs>
      <linearGradient
        id="paint0_linear_247_41620"
        x1="30"
        y1="13"
        x2="30"
        y2="47.7045"
        gradientUnits="userSpaceOnUse"
      >
        <GradientStops />
      </linearGradient>
    </defs>
  </SvgIcon>
);
