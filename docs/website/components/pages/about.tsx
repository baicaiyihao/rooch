import Link from 'next/link'
import { useRouter } from 'next/router'
import { InformationCircleIcon } from 'nextra/icons'

const About = () => {
  const currentLocale = useRouter().locale

  return (
    <Link href="/about">
      <button className="nx-h-7 nx-rounded-md nx-px-2 nx-text-left nx-text-xs nx-font-medium nx-text-gray-600 nx-transition-colors dark:nx-text-gray-400 hover:nx-bg-gray-100 hover:nx-text-gray-900 dark:hover:nx-bg-primary-100/5 dark:hover:nx-text-gray-50">
        <div className="nx-flex nx-items-center nx-gap-2">
          <InformationCircleIcon />
          {currentLocale === 'en-US' ? 'About' : '关于我们'}
        </div>
      </button>
    </Link>
  )
}

export default About
