export type SegmentId = string;

export enum SegmentType {
  LABEL = 'Label',
  DETECTION = 'Detection'
}

export class Segment {
  constructor(private _id: SegmentId | undefined, public from: number, public to: number, segmentType = SegmentType.LABEL) {
    if(this._id === undefined) {
      throw new Error('id is undefined');
    }
    if(isNaN(+from)) {
      throw new Error('from can`t be NaN');
    }
    if(isNaN(+to)) {
      throw new Error('to can`t be NaN');
    }
    this._segmentType = segmentType;
  }
  
  get id(): SegmentId { return this._id; }
  set id(value) { this._id = value; }

  get middle() { return (this.from + this.to) / 2; }

  get length() {
    return Math.max(this.from, this.to) - Math.min(this.from, this.to);
  }

  expandDist(allDist: number, portion: number): Segment {
    let p = Math.round(this.middle - allDist * portion / 2);
    let q = Math.round(this.middle + allDist * portion / 2);
    p = Math.min(p, this.from);
    q = Math.max(q, this.to);
    return new Segment(this._id, p, q);
  }

  equals(segment: Segment) {
    return this._id === segment._id;
  }

  // TODO: remove this and make original inheritence
  _segmentType: SegmentType
  get segmentType(): SegmentType { return this._segmentType; }
  set segmentType(value: SegmentType) { this._segmentType = value; }

  toObject() {
    return {
      id: this.id,
      from: this.from,
      to: this.to,
      segment_type: 'Label'
    }
  }

  static fromObject(obj: any) {
    return new Segment(
      obj.id,
      obj.from,
      obj.to,
      obj.segment_type
    );
  }

}