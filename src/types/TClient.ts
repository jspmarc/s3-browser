export default class Client {
  constructor(
    public accessKeyId: string,
    public bucketName: string,
    public secretAccessKey: string,
    public endpoint: string,
    public region: string,
    public isPathStyle: boolean
  ) {}

  generateUrl() {
    if (this.isPathStyle) return `${this.endpoint}/${this.bucketName}/`
    else if (this.endpoint.indexOf('aws') !== -1)
      return `https://${this.bucketName}.s3-${this.region}.${this.endpoint}/`
    else return `https://${this.bucketName}.${this.region}.${this.endpoint}/`
  }
}
