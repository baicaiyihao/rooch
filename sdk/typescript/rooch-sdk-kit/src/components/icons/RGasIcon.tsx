// Copyright (c) RoochNetwork
// SPDX-License-Identifier: Apache-2.0

import { ComponentProps } from 'react'

export function RGasIcon(props: ComponentProps<'svg'>) {
  return (
    <svg
      xmlns="http://www.w3.org/2000/svg"
      viewBox="0 0 500 500"
      width={24} // Set default width
      height={24} // Set default height
      {...props}
    >
      <circle
        cx="250"
        cy="250"
        r="250"
        fill="#b2ff04" // Inline styling for circle fill
      />
      <path
        fill="#006840" // Inline styling for path fill
        d="M346.39,403.15c-26.17-23.18-51.58-45.69-77.73-68.86-.13,2.58-.28,4.18-.29,5.77-.19,30.24-.36,60.48-.54,90.73q-.04,6.48-6.74,6.45c-8.78-.05-17.57-.27-26.35-.07-3.46,.08-4.36-.83-4.32-4.3,.33-31.62,.46-63.23,.64-94.85,0-1.05,.01-2.09,.02-4.12-26.46,22.99-52.29,45.44-78.06,67.84-1.6-.72-.96-1.95-.96-2.88,.04-14.97,.2-29.94,.17-44.9,0-2.36,.75-3.86,2.52-5.42,28.99-25.57,57.91-51.22,86.85-76.84,.67-.59,1.45-1.07,2.4-1.76-1.89-1.66-3.64-1.09-5.18-1.1-27.12-.2-54.24-.43-81.36-.41-3.91,0-4.88-1.05-4.74-4.83,.34-9.46,.42-18.94,.16-28.4-.11-3.83,1.29-4.39,4.68-4.35,26.96,.28,53.93,.38,80.9,.54,1.68,0,3.37,.02,5.83,.03-2.08-3.59-5.06-5.5-7.53-7.74-27-24.56-54.06-49.05-81.17-73.49-1.49-1.35-2.15-2.64-2.13-4.66,.17-16.03,.23-32.07,.33-49.24,26.24,23.34,51.86,46.12,78.27,69.61,.01-2.46,.02-4.06,.03-5.65,.18-31.01,.43-62.01,.45-93.02,0-3.71,1.1-4.55,4.65-4.44,9.39,.29,18.8,.42,28.19,.14,4.06-.12,4.69,1.28,4.65,4.93-.33,30.7-.44,61.4-.62,92.1,0,1.63-.02,3.27-.04,6.19,26.52-23.13,52.16-45.5,77.98-68.01,1.04,1.6,.64,2.99,.64,4.27-.05,13.9-.29,27.8-.13,41.7,.05,3.86-1.14,6.41-4.05,8.97-28.31,24.95-56.48,50.05-84.69,75.1-.78,.69-1.52,1.41-2.45,2.27,1.66,1.71,3.6,1.03,5.27,1.04,26.96,.21,53.93,.41,80.9,.44,3.42,0,4.77,.6,4.62,4.41-.37,9.61-.37,19.24-.19,28.86,.06,3.38-.77,4.35-4.31,4.31-27.12-.32-54.24-.4-81.36-.56-1.64,0-3.28-.02-6.14-.04,8.77,7.94,16.64,15.09,24.54,22.22,21.63,19.52,43.26,39.04,64.91,58.53,1.08,.97,1.81,1.89,1.8,3.46-.14,16.32-.22,32.64-.32,50.05Z"
      />
    </svg>
  )
}